#NoEnv  ; Recommended for performance and compatibility with future AutoHotkey releases.
; #Warn  ; Enable warnings to assist with detecting common errors.
SendMode Input  ; Recommended for new scripts due to its superior speed and reliability.
SetWorkingDir %A_ScriptDir%  ; Ensures a consistent starting directory.
#SingleInstance, force
SetTitleMatchMode, RegEx
SetTitleMatchMode, Fast

If WinExist(A_Args[1]) ; propresenter
{   ; if propresenter isn't active, switch and send clicker forward
    if !WinActive(A_Args[1]){
        WinActivate
        Send A_Args[2]
    }
}