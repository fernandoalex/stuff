local event_system = {
    callbacks = {}
}

function event_system:register(event_name, callback)
    self.callbacks[event_name] = self.callbacks[event_name] or {}
    table.insert(self.callbacks[event_name], callback)
end

function event_system:trigger(event_name, ...)
    if self.callbacks[event_name] then
        for _, callback in ipairs(self.callbacks[event_name]) do
            callback(...)
        end
    end
end

event_system:register("foo", function()
    print("bar1")
end)

event_system:trigger("foo")

event_system:register("foo", function()
    print("bar2")
end)

event_system:trigger("foo")
