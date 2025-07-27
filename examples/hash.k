print("Using strings as hash keys")

hash = Hash()
hash.insert("key1", "value1")

print(hash.get("key1"))

print("Using instances as hash keys")

class Test
end

test_instance = Test()

hash.insert(test_instance, "instance_value")

print(hash.get(test_instance))
