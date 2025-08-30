local attempts = 0

while attempts <= 10 do -- Loops until this ran 10 times
    openApp("notepad.exe") -- Opens notepad

    waitUntilAppOpen("notepad.exe", 100000) -- Waits until notepad opens for 100s

    sleep(1000) -- Waits 1s

    attempts = attempts + 1 -- Increases attempts by 1
end
