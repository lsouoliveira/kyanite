def test(a, b)
    print(a, b)
end

def test2(a, b)
    test(a, b)

    def test3(c, d)
        print(c, d)
    end

    test3(6, 7)
end

test(1, 2)
test2(3, 4)

print(true)
print(false)

a = String("a")

print(a)
