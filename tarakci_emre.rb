for i  in  0..5
    puts "Loop at i = #{i}:"
    if  i % 2 == 0 then
        next
    end
    j = 1
    while true
        puts "j: #{j}"
        j += 1
        break if j > 2
    end
end

