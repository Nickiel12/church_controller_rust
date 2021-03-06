#NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode Input  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir %A_ScriptDir%  ; Ensures a consistent starting directory.
#SingleInstance, force
SetTitleMatchMode, RegEx
SetTitleMatchMode, Fast

If WinExist(A_Args[1]) ; propresenter
{   ; if propresenter isn't active, switch and send clicker forward
    if (A_Args[3]) { ; 1 for from_hotkey 0 for from app
        if !WinActive(A_Args[1]){
            WinActivate
            sleep 200
            arg := A_Args[2]
            Send {%arg%}
        }
    } else {
        if !WinActive(A_Args[1]){
            WinActivate
            sleep 200
        }
        arg := A_Args[2]
        Send {%arg%}
    }
}