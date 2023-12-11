use std::{env, fs::File, sync::{Arc, mpsc}, thread, time::Instant, collections::HashMap, io::BufReader};

use anyhow::anyhow;
use serde::{Serialize, Deserialize};
use vibrato::{Dictionary, Tokenizer};


fn main() -> anyhow::Result<()> {
    println!("Loading twitter dump data...");
    let texts = get_twitter_texts()?;

    println!("Loading dict...");
    let dict = open_zstd_dictionary()?;

    let start = Instant::now();

    let n = num_cpus::get();
    
    let output = mecab(&texts, n, dict)?;

    let mut words_by_appearance: HashMap<&String, u32> = HashMap::new();

    for word in &output {
        match words_by_appearance.get_mut(&word) {
            Some(v) => *v = *&mut *v + 1,
            None => {
                words_by_appearance.insert(&word, 1);
            }
        };
    }

    let mut vec: Vec<(&&String, &u32)> = words_by_appearance.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!("==== RESULT ====");

    for (word, count) in vec.get(0..10).unwrap_or(&vec) {
        println!("「{}」{}回", word, count);
    }

    println!("{} ({}ms), {} threads", output.len(), start.elapsed().as_millis(), n);
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct TweetOuter{
    tweet: Tweet
}

#[derive(Serialize, Deserialize)]
struct Tweet{
    created_at: String,
    full_text: String
}

fn get_twitter_texts() -> anyhow::Result<Vec<String>> {

    let file = File::open(env::current_dir()?.join("./twitter").join("./tweets.json"))?;
    
    let reader = BufReader::new(file);

    let tweets: Vec<TweetOuter> = serde_json::from_reader(reader)?;

    let texts = tweets
        .iter()
        .filter(|t|t.tweet.created_at.ends_with("2023"))
        .map(|t|t.tweet.full_text.to_owned())
        .collect();

    Ok(texts)
}

fn mecab(texts: &Vec<String>, n: usize, dict: Dictionary) -> anyhow::Result<Vec<String>> {

    if texts.len() == 0 {
        return Err(anyhow!("texts len zero"));
    }

    let n = if n <= texts.len() { n } else {texts.len()};

    // 解析器を作る
    let tokenizer = Tokenizer::new(dict)
        .ignore_space(true)
        .unwrap()
        .max_grouping_len(24);

    // Arcで包む
    let tokenizer = Arc::new(tokenizer);

    // chunksを計算
    let chunk_size = (texts.len() + n - 1) / n;

    // スレッドから結果を受け取る用
    let (tx, rx) = mpsc::channel();

    // 分割して複数スレッドに渡す
    for texts in texts.chunks(chunk_size){
        let tokenizer = Arc::clone(&tokenizer);
        let texts = texts.to_owned();
        let tx = tx.clone();
        thread::spawn(move ||{
            let mut worker = tokenizer.new_worker();
            let texts: Vec<String> = texts.iter().flat_map(|text|{
    
                worker.reset_sentence(text);
                worker.tokenize();

                let tokenized: Vec<String> = worker
                    .token_iter()
                    .filter(|t|t.feature().contains(",固有名詞"))
                    .map(|t| t.surface().to_string())
                    .collect();

                tokenized
            })
            .collect();
            tx.send(texts).unwrap();
        });
    }

    // すべてのスレッドが返すまで待つ
    let mut c = 0;
    let mut result_vec = vec![];

    while c < n {
        let mut received = rx.recv().unwrap();

        println!("Collect! {}", c);
        
        result_vec.append(&mut received);

        c += 1;
    }

    return Ok(result_vec);
    
}

fn open_zstd_dictionary() -> anyhow::Result<Dictionary> {
    let basedir = "./";
    let dict_path = "ipadic-mecab-2_7_0/system.dic.zst";

    let dict_full_path = env::current_dir()?.join(basedir).join(dict_path);

    if !dict_full_path.exists() {
        panic!("dict file not found: {}", dict_full_path.to_string_lossy());
    }

    if !dict_full_path.exists() {
        panic!("not a file dict: {}", dict_full_path.to_string_lossy());
    }

    let reader = zstd::Decoder::new(File::open(dict_full_path)?)?;
    let dict = Dictionary::read(reader);

    return Ok(dict?);
}