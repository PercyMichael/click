import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [greeting, setGreet] = useState();
  const [message, setMessage] = useState("");
  const [mouseClickMessage, setMouseClickMessage] = useState("");

  const greet = async () => {
    const a = await invoke("greet");
    setGreet(a);
  };

  // Function to greet and show current date from the backend
  const greetAndShowDate = async () => {
    try {
      // Invoke the greet_and_date function which returns the greeting and current date
      const result = await invoke("greet_and_date");
      // Set the message in the state
      setMessage(result);
    } catch (error) {
      console.error("Error invoking greet_and_date:", error);
    }
  };

  useEffect(() => {
    // Listen for mouse click events from the backend
    const unlisten = listen("mouse-click", (event) => {
      setMouseClickMessage(event.payload);
    });

    // Cleanup the listener on component unmount
    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div>
      <h1>{greeting}</h1>
      <h1>{message}</h1>
      <h1>{mouseClickMessage}</h1>
      <button onClick={greet}>Greet Me</button>
      <button onClick={greetAndShowDate}>Greet and Show Current Date</button>
    </div>
  );
}

export default App;
