module Stats
end

module Stats::Median
    def self.get_median(list)
        length = list.length
        
        if (length % 2 == 0)
            middle_index = length / 2
            middle_number = list[middle_index]
            next_middle_number = list[middle_index + 1]
    
            return (next_middle_number + middle_number) / 2
        end
    
        middle_index = ((length - 1) / 2)
    
        list[middle_index]
    end
end

module Stats::Mode
    def self.get_mode(list)
        counts_map = Hash.new

        list.each do |n|
            if counts_map.has_key?(n)
                counts_map[n] += 1
            else
                counts_map[n] = 1
            end
        end

        counts_map.max_by{|k,v| v}[0]
    end
end