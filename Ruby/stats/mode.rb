module Stats
    module Mode
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
end