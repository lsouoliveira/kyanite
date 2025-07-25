def test
    print("I'm inside a thread")
end

a = Thread(test)

a.start()
a.join()
