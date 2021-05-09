import React, { useState, useEffect } from "react";
import debounce from "debounce";
import { invoke } from "@tauri-apps/api/tauri";

type ItemProps = {
  image: string;
  title: string;
};
function Item(props: ItemProps) {
  return (
    <div className="h-72 flex flex-col rounded shadow-md cursor-pointer">
      <img className="h-60" src={props.image} />
      <span className="text-center pt-2">{props.title}</span>
    </div>
  );
}

const search = debounce(async (text: string) => {
  if (text != "") {
    return await invoke("gogoanime_search", { text });
  }

  return [];
}, 300);

function App() {
  const [result, setResult] = useState([
    {
      cover_image_url:
        "https://s4.anilist.co/file/anilistcdn/media/anime/cover/large/bx114535-y3NnjexcqKG1.jpg",
      title: "To Your Eternity",
    },
  ]);
  const [text, setText] = useState("");

  useEffect(() => {
    const res = search(text);

    if (res) {
      res.then(setResult);
    }
  }, [text]);

  return (
    <div className="p-10 flex flex-col items-center h-full">
      <input
        className="bg-gray-100 rounded-lgm p-2 w-1/3"
        placeholder="Search..."
        onChange={(ev) => setText(ev.target.value)}
        value={text}
      />
      <div className="mt-10 w-full h-full px-80 grid grid-cols-6 gap-6">
        {result.map((a, i) => (
          <Item key={i} image={a.cover_image_url} title={a.title} />
        ))}
      </div>
    </div>
  );
}

export default App;
