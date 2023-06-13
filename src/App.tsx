import {useEffect ,Suspense, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import ReactLoading from 'react-loading';
import "./App.css";

function App() {
  const [text, setText] = useState("");
  const [topnoun, setTopnoun] = useState("");
  const [countNoun, setCountnoun] = useState(""); 
  const [isLoading, setIsLoading] = useState(false);
  const [flag, setFlag] = useState(false);

  //名詞の数
  async function get_count_noun(): Promise<React.SetStateAction<string>>{
    var count = await invoke<string>("get_count_noun", {text});
    return count;
  }
  //名詞の多かった上位5つ
  async function get_top_noun(): Promise<React.SetStateAction<string>>{
    return await invoke("get_top_noun", {text});
  }
  //名詞情報の取得
  async function get_noun_information(){
    setIsLoading(true); 

    try{
      const count = get_count_noun();
      const top = get_top_noun();

      //promisのラップをむく
      await count.then((countNoun) => {
        const newState: React.SetStateAction<string> = countNoun;
        setCountnoun(newState);
      });
      await top.then((topnoun) => {
        const newState: React.SetStateAction<string> = topnoun;
        setTopnoun(newState);
      });
    } catch{}
    setIsLoading(false);
  }

  return (
    <>
    <h1>Word Count</h1>
    <textarea
      id="textarea"
      placeholder="ここに入力"
      value={text}
      onChange={(e) => {
        setText(e.target.value);
      }}
    />
    <button onClick={(e) => {
      get_noun_information();    
    }}>submit</button>

    <p>単純な文字数：{text.length}</p>
    <p>改行なしの文字数：{text.replaceAll("\n","").length}</p>

    {isLoading ? <ReactLoading type={"spin"} color={"black"} height={'20%'} width={'20%'} /> : 
    <div>
      <p>名詞の数：{countNoun}</p>
      <p>名詞上位5個：{topnoun}</p>
    </div>
    }
    </>
  );
}

export default App;
