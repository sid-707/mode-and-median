module Stats
    module Median
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
end