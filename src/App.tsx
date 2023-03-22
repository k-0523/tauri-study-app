import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import MarkdownPreview from '@uiw/react-markdown-preview';
import "./App.css";

function App() {
  const [studyLog, setStudyLog] = useState("");
  const [beforeStudyLog, setBeforeStudyLog] = useState("");
  const [today, setToday] = useState('');
  const [beforeDay, setBeforeDay] = useState('');
  const [isPreviewMode, setIsPreviewMode] = useState(false);

  useEffect(() => {
    setToday(formatDate(new Date()));
    setBeforeDay(getBeforeNdays(5));

    fetchContent(today, true);
    fetchContent(beforeDay, false);
  }, []);

  // 内容を取得
  // TODO: エラーハンドリング
  const fetchContent = async (date: String, isToday: Boolean) => {
    await invoke("get", { date: date }).then(
      response => {
        if (isToday) {
          setStudyLog(response.content);
        } else {
          setBeforeStudyLog(response.content);
        }
      }
    );  }

  // yyyy-mm-dd形式の日付を取得
  const formatDate = (dt: any) => {
    var y = dt.getFullYear();
    var m = ('00' + (dt.getMonth()+1)).slice(-2);
    var d = ('00' + dt.getDate()).slice(-2);
    return (y + '-' + m + '-' + d);
  }

  // 数日前のデータを取得する
  const getBeforeNdays = (n: number) => {
    var dt = new Date();
    dt.setDate(dt.getDate() - n);
    return formatDate(dt);
 }

  // 保存処理
  // TODO: エラーハンドリング
  const save = async (content: String) => {
    await invoke("save", { content: content, date: today });
    setStudyLog(content);
  }

  return (
    <div className="container">
      <div className="left-panel">
        <p className="date">
          5日前（{beforeDay}）
        </p>
        <div style={{ paddingLeft: '10px', paddingRight: '20px' }}>
          <MarkdownPreview className="markdown" source={beforeStudyLog} />
        </div>
      </div>
      <div className="right-panel">
        <p className="date">
          今日（{today}）
        </p>
        <div style={{ paddingLeft: '10px', paddingRight: '20px' }}>
          {
            isPreviewMode ?
            <MarkdownPreview className="markdown-today" source={studyLog} /> :
            <textarea defaultValue={studyLog} onChange={(e) => save(e.target.value)}></textarea>
          }
        </div>
        <div className="upper">
          <label htmlFor="check">
            Preview：
          </label>
          <input
            type="checkbox"
            id="check"
            checked={isPreviewMode}
            onChange={() => setIsPreviewMode(prevState => !prevState)}
          />
        </div>
      </div>
    </div>
  );
}

export default App;