```fish
# Produce messages
for i in (seq 20 30)
    echo $i | kcat -P -l -b localhost:29092 -t numbers
end

# Consume messages
kcat -C -b localhost:29092 -t numbers
```
