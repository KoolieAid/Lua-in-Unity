using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;

public class Controlled : MonoBehaviour
{
    private IntPtr lua;
    
    // Start is called before the first frame update
    void Start()
    {
        unsafe
        {
            lua = Lua.InitLua(Lua.DebugWrapper);
        }
        
        Debug.Log("thing");
    }

    private void OnDestroy()
    {
        Lua.DestroyLua(lua);
        
        
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetKeyDown(KeyCode.Y))
        {
            var code = "debuglog(\'fuck\')";
            Exec(code);
        }
    }

    public void Exec(string code)
    {
        var charptr = Marshal.StringToHGlobalAnsi(code);

        unsafe
        {
            Lua.Execute(lua, (sbyte*)charptr);
        }
        
        Marshal.FreeHGlobal(charptr);
    }
}
