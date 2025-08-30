if not isAppOpen("chrome.exe") then --Starts if statement and checks if chrome is not open
    messageBox("Exiting", "Exiting with error code 5") --Message with title exiting
    exit(5) --Exits with error code 5
end --Ends the if statement

openApp("chrome.exe") --Opens chrome