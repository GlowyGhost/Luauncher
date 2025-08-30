if system == "Windows" then --If user is on windows
    runCommand("notepad.exe") --Runs command "notepad.exe" (opens notepad)

elseif system == "Linux" then --If user is on linux
    runCommand("gedit") --Runs command "gedit" (opens gedit)

elseif system == "MacOS" then --If user is on MacOS
    runCommand("TextEdit") --Runs command "TextEdit" (opens TextEdit)
end