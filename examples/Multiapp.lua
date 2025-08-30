openApp("calc.exe") -- Opens calculator.exe

waitUntilAppOpen("calc.exe", 100000) -- Waits for the clac to open for 100s

sleep(2000) -- Waits for 2s

closeApp("calc.exe") -- Closes calc

waitUntilAppClose("calc.exe") -- Waits for the clac to close

sleep(2000) -- Waits for 2s

openApp("notepad.exe") -- Opens notepad.exe

waitUntilAppOpen("notepad.exe", 100000) -- Waits for notepad to open for 100s
