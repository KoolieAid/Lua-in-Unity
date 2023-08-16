using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using AOT;
using UnityEngine;

public unsafe class Lua
{
    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate void AcceptsString(byte* @byte);
    
    #if UNITY_EDITOR
    private const string DllName = "Assets/Plugins/liblua.dll";
    #else
    private const string DllName = "/Plugins/x86_64/liblua.dll:";
    #endif

    [DllImport(DllName, EntryPoint = "init_lua")]
    public static extern IntPtr InitLua(AcceptsString debug);

    [DllImport(DllName, EntryPoint = "destroy_lua")]
    public static extern void DestroyLua(IntPtr lua);

    [DllImport(DllName, EntryPoint = "register_function")]
    public static extern void RegisterFunction(IntPtr lua, sbyte* name, Action func);

    [DllImport(DllName, EntryPoint = "execute")]
    public static extern void Execute(IntPtr lua, sbyte* chunk);

    [DllImport(DllName, EntryPoint = "free_string")]
    public static extern void FreeString(byte* str);

    [MonoPInvokeCallback(typeof(AcceptsString))]
    public static void DebugWrapper(byte* str)
    {
        var s = new string((sbyte*)str);
        FreeString(str);
        
        Debug.Log($"{s}");
    }
}
