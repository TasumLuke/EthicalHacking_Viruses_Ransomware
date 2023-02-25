def replicate
  File.open(__FILE__, "w") do |f|
    f.puts self.to_s
  end
end

replicate
