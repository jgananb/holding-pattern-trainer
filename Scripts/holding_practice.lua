-- Holding Pattern Practice Script for X-Plane 11
-- Solo lee datos de X-Plane y los escribe a JSON
-- La app Rust maneja toda la lógica

-- Función para escribir datos a archivo JSON
function write_xplane_data()
    -- Leer datos de X-Plane
    local vor_freq = get("sim/cockpit2/radios/actuators/nav1_frequency_hz") or 0
    local vor_id = get("sim/cockpit2/radios/indicators/nav1_nav_id") or ""

    local lat = get("sim/flightmodel/position/latitude") or 0
    local lon = get("sim/flightmodel/position/longitude") or 0
    local alt = get("sim/flightmodel/position/elevation") or 0
    local heading = get("sim/cockpit2/gauges/indicators/heading_AHARS_deg_mag_pilot") or 0
    local groundspeed_ms = get("sim/flightmodel/position/groundspeed") or 0
    local groundspeed = groundspeed_ms * 1.94384  -- Convertir m/s a nudos

    -- Buscar VOR si hay uno sintonizado
    local vor_lat = 0
    local vor_lon = 0

    if vor_id ~= "" and vor_id ~= nil and vor_id ~= "    " and vor_freq > 0 then
        local vor_ref = XPLMFindNavAid(nil, nil, lat, lon, vor_freq, xplm_Nav_VOR)
        if vor_ref ~= nil and vor_ref ~= 0 then
            local nav_type, v_lat, v_lon, vor_height, vor_freq_check, vor_heading, vor_id_check, vor_name = XPLMGetNavAidInfo(vor_ref)
            if v_lat and v_lon then
                vor_lat = v_lat
                vor_lon = v_lon
            end
        end
    end

    -- Escribir JSON con datos básicos de X-Plane
    local file = io.open(SCRIPT_DIRECTORY .. "xplane_data.json", "w")
    if file then
        file:write("{\n")
        file:write(string.format('  "vor_id": "%s",\n', vor_id))
        file:write(string.format('  "vor_freq": %d,\n', vor_freq))
        file:write(string.format('  "vor_lat": %.6f,\n', vor_lat))
        file:write(string.format('  "vor_lon": %.6f,\n', vor_lon))
        file:write(string.format('  "aircraft_lat": %.6f,\n', lat))
        file:write(string.format('  "aircraft_lon": %.6f,\n', lon))
        file:write(string.format('  "aircraft_alt": %.1f,\n', alt))
        file:write(string.format('  "aircraft_heading": %.1f,\n', heading))
        file:write(string.format('  "aircraft_groundspeed": %.1f\n', groundspeed))
        file:write("}\n")
        file:close()
    end
end

-- Escribir datos cada segundo
do_often("write_xplane_data()")

-- Mensaje de inicio
logMsg("Holding Practice Data Writer loaded!")
