# Lua-in-Unity
Proof of concept on having a Lua runtime in Unity.

#
Since this is just a proof of concept, it has no means to be a full game, nor do I plan to.

# How it works
Using the Rust language, the Lua runtime is added into Unity. It is done by having Rust act as a middleman between the runtime, and the C# / Unity runtime.

# Why rust
I used Rust because it provides:
- Memory Safefy
  - Prevents crashes while game is playing
- Low Memory Usage
- Small Binary Size

# Usage
<p> Usage that has Lua code is provided in <code>Controlled.cs</code>. Currently, it only has a global function in the Lua runtime to call Debug.Log in unity. This is done with `debuglog` in Lua. </p>
<p> Registering a function is done using <code>RegisterFunction</code> in <code>Lua.cs</code></p>

