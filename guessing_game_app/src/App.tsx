import './App.css';
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core'

export default function App() {

  const randomNumber = Math.floor(Math.random() * 100);

  const [guessNumber, setGreet] = useState<String>("");
  const [error, setError] = useState<String>("");

  async function handleClick(formData: FormData) {
    try {
      const guess = formData.get("guess");
      setError("");
      if (typeof (guess) !== 'string' || guess === null || guess === "") throw new Error("guess could be empty or null");
      const resultFromRust = await invoke<string>("guessing_game", { inputString: guess.trim(), randomNumber: randomNumber });
      if (typeof resultFromRust !== "string" || resultFromRust.trim() === "") {
        throw new Error("Rust returned an empty response.");
      };
      setGreet(resultFromRust);
    } catch (error: unknown) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.log(error);
      setError(errorMessage);
    };
  };

  return (
    <div>
      <form action={handleClick}>
        <input type="text" name="guess" placeholder="type a number" required />
        <button>Guess</button>
      </form>
      {guessNumber && <p>{guessNumber}</p>}
      {error && <p role="alert">{error}</p>}
    </div>
  );
};