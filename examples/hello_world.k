a = "Hello, world!"

print(a)

print("Hello, world!")
print("Hello, world!")

def print2(text)
    print(text)
    print(text)
end

print2("Hello, world!")

class Test
    name = "Test"
end

class Printer
    name = Test()

    def print(arg)
        print(self.name)
    end
end

a = Printer()

a.print("test")
