extends Node3D  # Ensure this script is attached to a Node3D

# Constants
const DAY := 24
const HOUR := 60
const TIMEOUT := 100
const START_HOUR := 22

# Variables
var tick := 3
var hour := START_HOUR
var minute := 0
var timeout := 0
var rng := RandomNumberGenerator.new()

@onready var label := $clock/Label3D as Label3D  # Correct path relative to `other_clock`

func _ready():
	rng.randomize()  # Seed the random number generator
	print("Label3D found:", label != null)  # Debugging: Check if Label3D is found
	update_time()

func _process(delta: float):
	if timeout >= TIMEOUT:
		timeout = 0
		increment_time()
		update_time()
	else:
		timeout += 1

func increment_time():
	minute += rng.randi_range(0, tick)  # Increment minutes by a random value (0 to tick)

	if minute >= HOUR:
		hour += 1
		minute = 0

	if hour >= DAY:
		hour = 0
		minute = 0

func update_time():
	if label != null:  # Ensure label is not null before updating
		label.text = standard_time()  # Update the Label3D text
	else:
		print("Label3D is null! Check the node path.")  # Debugging: Warn if label is null

func military_time() -> String:
	return "%02d:%02d" % [hour, minute]  # Format as HH:MM

func standard_time() -> String:
	var hour_12 = hour % 12
	var suffix = "am" if hour < 12 else "pm"
	return "%02d:%02d %s" % [hour_12, minute, suffix]  # Format as HH:MM am/pm
