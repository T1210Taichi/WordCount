use std::fs::File;
use vibrato::{Dictionary, Tokenizer};
use vibrato::tokenizer::worker::Worker;
use std::collections::HashMap;

//vibratoのtokenizerを返す
fn mecab() -> Tokenizer{
    let home_dir = "./"; // 辞書ファイルを置いた場所に書き換えて。
    let dict_path = "ipadic-mecab-2_7_0/system.dic.zst";
    let dict_full_path = format!("{}/{}", home_dir, dict_path);

    // 辞書ファイルのロード
    let reader = zstd::Decoder::new(File::open(dict_full_path).unwrap()).unwrap();
    let dict = Dictionary::read(reader).unwrap();

    // トークナイザーの生成
    let tokenizer = Tokenizer::new(dict)
        .ignore_space(true).unwrap()
        .max_grouping_len(24);

    return tokenizer;
}

//textの内、 「名詞」の配列を返す。
pub fn get_noun(text:String) -> Vec<String>{
    let tokenizer= mecab();//tokenizer
    let mut worker = tokenizer.new_worker();
    worker.reset_sentence(text);//reset
    worker.tokenize(); // 形態素解析の実行。mutable self

    let mut count:Vec<String> = Vec::new();

    for token in worker.token_iter(){ 
        let words:Vec<&str> = token.feature().split(',').collect();
        let subwords:Vec<&str> = words[0].split('-').collect();
        if subwords[0] == "名詞"{
            count.push(token.surface().to_string());
        }
    }

    return count;
}

//textの「名詞」の配列の内、上位n個の名詞を返す。
pub fn get_top_noun(text:String) -> Vec<String>{ 
    let mut count_map: HashMap<String, usize> = HashMap::new();
    let mut top:Vec<String> = Vec::new();

    let noun_vec:Vec<String> = get_noun(text);

    //要素の出現回数をカウント
    for token in noun_vec{
        let count = count_map.entry(token).or_insert(0);
        *count += 1;
    }
    //カウント順にソート
    let mut sort_vec:Vec<_> = count_map.into_iter().collect();
    sort_vec.sort_by(|x, y| y.1.cmp(&x.1));

    // 上位5つの要素をVec<String>として返す
    sort_vec.truncate(5);
    for v in sort_vec{
        top.push(v.0 + "(" + v.1.to_string().as_str() + ")");
    }

    return top;
}