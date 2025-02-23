extends Node3D

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

@onready var label := $Clock/Time as Label3D  # Ensure the path to your Label3D node is correct

func _ready():
	rng.randomize()  # Seed the random number generator
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
	label.text = military_time()  # Update the Label3D text

func military_time() -> String:
	return "%02d:%02d" % [hour, minute]  # Format as HH:MM

func standard_time() -> String:
	var hour_12 = hour % 12
	var suffix = "am" if hour < 12 else "pm"
	return "%02d:%02d %s" % [hour_12, minute, suffix]  # Format as HH:MM am/pm
