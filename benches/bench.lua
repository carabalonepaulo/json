local bench = {}

local clock = os.clock

local function fmt_sec(v)
  return string.format("%.3fs", v)
end

local function fmt_ms(v)
  return string.format("%.4fms", v * 1000)
end

local function run(name, func, iterations)
  iterations = iterations or 100000

  for i = 1, math.min(iterations, 100) do
    func()
  end

  collectgarbage("collect")

  local min, max = math.huge, 0
  local total_start = clock()

  for i = 1, iterations do
    func()
  end

  local total_time = clock() - total_start
  local avg = total_time / iterations
  local ops_sec = total_time > 0 and (iterations / total_time) or 0

  local result = string.format(
    "%-15s | Total: %-8s | Avg: %-10s | Ops/sec: %.2f",
    name, fmt_sec(total_time), fmt_ms(avg), ops_sec
  )
  return ops_sec, result
end

function bench.compare(group, cases, iterations)
  print(string.rep("-", 80))
  print(string.format("%s: %d iterations", group, iterations))
  print(string.rep("-", 80))

  local results = {}
  for name, func in pairs(cases) do
    table.insert(results, { run(name, func, iterations) })
  end

  table.sort(results, function(a, b)
    return a[1] > b[1]
  end)

  for _, row in ipairs(results) do
    print(row[2])
  end

  print(string.rep("-", 80) .. "\n")
end

return bench
