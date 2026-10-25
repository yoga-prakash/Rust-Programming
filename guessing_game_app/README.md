<h1>Guessing Game</h1>
<p>Created a simple and basic guessing game that generates a number and compared with user guessed number and shows correct or not.</p>
<p>Stack: <em>TypeScript, React, Rust and Tauri</em>.</p>
<p>start using this command - 
<kbd><code>vp exec tauri dev</code></kbd> in the terminal of your prefered code editor.
</p>
<p>Some files or folders might be blocked by defender so some error might occur when building, as in total of 356 only 354 things will be built.</p>
<p>After building, a new window will open and user can type inside the input field and press guess button to trigger the tauri command - 
<pre><code class="language-rust">
#[tauri::command]
fn guessing(input_params) {
  ...snippets
}
</code></pre>
</p>
<p>This code will check the inputed string and generated random number using -
<pre><code>
std::cmp::Ordering; // Enum
</code></pre> and 
<pre><code>
match // keyword
</code></pre>
among the enum 
<pre><code>
Ordering::Less => {}
Ordering::Greater => {}
Ordering::Equal => {}
</code></pre>
if any match that particular enum's function code will be run.
</p>