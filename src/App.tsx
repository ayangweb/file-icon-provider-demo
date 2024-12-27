import { invoke } from "@tauri-apps/api/core";
import { Flex, Image, Input, message } from "antd";
import { useState } from "react";

const App = () => {
  const [url, setUrl] = useState("");

  return (
    <Flex vertical gap="middle">
      <Input.Search
        enterButton="Get Icon"
        placeholder="Enter a path"
        onSearch={async (value) => {
          if (!value) {
            return message.warning("Please enter a path");
          }

          const bytes = await invoke<Uint8Array>("plugin:icon|get_icon", {
            path: value,
            size: 256,
          });

          const blob = new Blob([new Uint8Array(bytes)], { type: "image/png" });
          const url = URL.createObjectURL(blob);
          setUrl(url);
        }}
      />

      {url && <Image src={url} width={128} />}
    </Flex>
  );
};

export default App;
