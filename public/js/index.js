let play_result; 
const gridWidth = 100;
const gridColors = ["#E7D4C0","#429E9D","#658EA9","#E98973","#F73019"];

function overlay_off() {
	document.getElementsByClassName("overlay")[0].style.display = "none";	
}

function set_loading() {
	document.getElementById("run").style.display = "none";
	document.getElementById("loading").style.display = "block";
	document.getElementsByClassName("overlay")[0].classList.remove("playagain");
}

function set_playagain() {
	document.getElementById("loading").style.display = "none";
	let overlay = document.getElementsByClassName("overlay")[0];
	let button = document.getElementById("run");
	overlay.style.display = "block";
	overlay.classList.add("playagain");
	button.style.display = "block";
	button.innerText = "Play again!";
}

function init() {
	init_grid();
	get_players();
}

function init_grid() {
	let grid = document.getElementsByClassName("grid-container")[0];
	for (let i = 0; i < 10000; i++) {
		let item = document.createElement("div");
		item.classList.add("grid-item");
		item.setAttribute("id", "item-" + i);
		grid.appendChild(item);
	}
}

function reset_grid() {
	let items = document.getElementsByClassName("grid-item");
	[...items].forEach(item => {
		item.style.backgroundColor = gridColors[0];
	});
}

function get_players() {
	$.ajax({
		url: "players",
		success: function(data, status, xhr) {
			let playerSelect = document.getElementsByClassName("player-select");
			const players = JSON.parse(data);
			Array.from(playerSelect).forEach(select => {
				players.forEach(player => {
					let option = document.createElement("option");
					option.setAttribute("value", player);
					option.innerHTML = player.substring(0, player.indexOf('.'));
					select.appendChild(option);
				})
			});
		}
	});
}

function run_game() {
	reset_grid();
	set_loading();
	const player1 = document.getElementById("p1-Select").value;
	const player2 = document.getElementById("p2-Select").value;
	$.get("run",
		{ p1: player1, p2: player2 },
		async function(data, status, xhr) {
			overlay_off();
			await Promise.all(run_replay(data));
			set_playagain();
		}
	);
}

function run_replay(data) {
	const array = JSON.parse(data);
	let grid = $("div.grid-item");
	return array.map((state, index) => _set_state(grid, state, index));
}

function _set_state(grid, state, stateNum) {
	return new Promise((resolve, reject) => {
		setTimeout(function () {
			state.forEach((line, lnIndex) => {
				_set_grid_line(grid, lnIndex, line);
			});
			resolve(1);
		}, 10 * stateNum)
	});
}

function _set_grid_line(grid, lnIndex, line)
{
	const items = line.split('').map(item => {
		switch (item) {
			case 'O':
				return 1;
			case 'o':
				return 2;
			case 'X':
				return 3;
			case 'x':
				return 4;
			default:
				return 0;
		}
	});
	let itemNum;
	items.forEach((item, index) => {
		itemNum = lnIndex * gridWidth + index;
		grid[itemNum].style.backgroundColor = gridColors[item];
	});
}

window.onload = init;
