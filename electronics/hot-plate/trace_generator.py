import os
import math

TRACE_WIDTH_MM = 1.25
TRACE_SPACING_MM = 1.9
TRACE_START_X_MM = 153
TRACE_PAD_OFFSET_X_MM = -6
TRACE_START_Y_MM = 148
BOARD_WIDTH_MM = 88
BOARD_HEIGHT_MM = 96

def update_trace(file_path):
    print("Modifying the file: " + file_path)

    file_content = ""
    with open(file_path, "r") as file:
        file_content = file.readlines()

    print("File before: " + '\n'.join(file_content))
    with open(file_path, "w") as file:
        LINES_TO_DELETE = "segment"
        was_the_trace_generated = False
        
        for line in file_content:  
            if not LINES_TO_DELETE in line:
                file.write(line)
            elif not was_the_trace_generated:
                was_the_trace_generated = True

                trace = generate_trace()
                file.write(trace)

def generate_trace():
    trace = ""

    trace_points = [
        (TRACE_START_X_MM + TRACE_PAD_OFFSET_X_MM, TRACE_START_Y_MM),
        (TRACE_START_X_MM + TRACE_PAD_OFFSET_X_MM - BOARD_WIDTH_MM / 2, TRACE_START_Y_MM),
        (TRACE_START_X_MM + TRACE_PAD_OFFSET_X_MM - BOARD_WIDTH_MM / 2, TRACE_START_Y_MM - BOARD_HEIGHT_MM),
        (TRACE_START_X_MM + BOARD_WIDTH_MM / 2 + TRACE_SPACING_MM, TRACE_START_Y_MM - BOARD_HEIGHT_MM)
    ]
    rows_count = math.floor(BOARD_HEIGHT_MM / TRACE_SPACING_MM) - 1
    # The rows count must be an even number for this small script to work correctly (the last row of the trace must be on the right)
    if rows_count % 2 == 0:
        rows_count -= 1
        
    is_on_the_right = True
    for row in range(rows_count):
        point = trace_points[-1]
        is_last_row = row == rows_count -1
        new_y = TRACE_START_Y_MM if is_last_row else point[1] + TRACE_SPACING_MM
        point = (point[0], new_y)
        trace_points.append(point)

        x_variation = BOARD_WIDTH_MM - TRACE_PAD_OFFSET_X_MM - TRACE_SPACING_MM
        if is_last_row:
            x_variation = (BOARD_WIDTH_MM + TRACE_PAD_OFFSET_X_MM) * -0.5
        elif is_on_the_right:
            x_variation *= -1
            
        point = (point[0] + x_variation, point[1])
        trace_points.append(point)
        
        is_on_the_right = not is_on_the_right

    print("Generating the trace with a width of " + str(TRACE_WIDTH_MM) + "mm, with a spacing between each row of " + str(TRACE_SPACING_MM) + "mm, and with " + str(rows_count) + " rows") 

    segment = "  (segment (start {start_x} {start_y}) (end {end_x} {end_y}) (width " + str(TRACE_WIDTH_MM) + ") (layer \"F.Cu\") (net 1))\n"
    
    (start_x, start_y) = trace_points[0]
    for point in trace_points:
        trace += segment.format(start_x = start_x, start_y = start_y, end_x = point[0], end_y = point[1])
        (start_x, start_y) = point
    
    return trace

FILE_EXTENSION = ".kicad_pcb"

file_path = input("- Insert path of the .kicad_pcb file (leave blank to automatically search it): ")
if file_path == "":
    for file in os.listdir("./"):
        if file.endswith(FILE_EXTENSION):
            file_path = file

if not file_path.endswith(FILE_EXTENSION):
    file_path += FILE_EXTENSION
update_trace(file_path)
