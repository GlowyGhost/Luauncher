openApp("calc.exe") -- Opens calculator.exe

waitUntilAppOpen("calc.exe", 100000) -- Waits for the clac to open for 100s

sleep(2000) -- Let the user see the calculator for 2 seconds

closeApp("calc.exe") -- Closes the app

waitUntilAppClose("calc.exe") -- Waits for the app to close
