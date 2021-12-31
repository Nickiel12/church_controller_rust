#NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode Input  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir %A_ScriptDir%  ; Ensures a consistent starting directory.
#SingleInstance, force
SetTitleMatchMode, RegEx
SetTitleMatchMode, Fast

If WinExist(A_Args[2]) ;OBS
{   ; if obs isn't active, switch to it, and send the hotkey
    If !WinActive(A_Args[2]) {
        WinActivate
        sleep 200
    }
    arg := A_Args[3]
    Send {%arg%}
}

sleep 100

If WinExist(A_Args[1]) ; Propresenter
{   ; switch back to propresenter
    If !WinActive(A_Args[1]) {
        WinActivate
    }
}