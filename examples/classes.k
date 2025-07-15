# This is a comment
class MyClass
    attribute = "my_attribute"

    def constructor(attribute2)
        self.attribute2 = attribute2
    end

    def display
        self.attribute = "1"

        print("Attribute: " + self.attribute)
    end
end

my_class = MyClass("my_attribute2")
my_class.display()
