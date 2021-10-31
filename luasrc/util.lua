return {
  corpse_frame_sequence = function(die_frame, corpse_frame)
    result = { 1 }
      rep = math.ceil(60 / die_frame)
      for i = 1, corpse_frame do
        for _ = 1, rep do table.insert(result, i + 1) end
      end
    return result
  end,

  calc_corpse_time = function(die_frame, corpse_frame)
    return (corpse_frame * math.ceil(60 / die_frame) + 1) * die_frame
  end,

  create_base_unit = function()
    return {
      type = "unit",
      flags = {"placeable-player", "placeable-enemy", "placeable-off-grid"},
      subgroup = "enemies",
      ai_settings = { destroy_when_commands_fail = true, allow_try_return_to_spawner = true }
    }
  end
}
