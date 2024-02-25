import { Chessboard } from "./vendor/board/Chessboard.js";
import init, { Chess } from "./chess/chess_engine_demo.js";

class App {
    constructor() {
        this.chess = Chess.new();
        this.board = new Chessboard(document.getElementById("board"), {
            position: this.chess.to_fen(),
        });
        this.lastValidMove;
        this.gameState = "Ongoing";
        this.info = document.getElementById("info");

        this.board.enableMoveInput(this.handleInput.bind(this));
    }

    validateMoveInput(event) {
        const move = [event.squareFrom, event.squareTo];

        if (this.chess.validate_move(move)) {
            this.lastValidMove = move;
            return true;
        } else {
            this.lastValidMove = false;
            return false;
        }
    }

    handleMoveInput(event) {
        if (!this.lastValidMove) return false;

        this.gameState = this.chess.make_move(this.lastValidMove);
        event.chessboard.setPosition(this.chess.to_fen(), false);

        this.board.disableMoveInput();
        this.info.innerText = "Engine is thinking...";

        if (this.gameState !== "Ongoing") {
            this.handleGameOver();
            return false;
        }

        setTimeout(() => {
            this.gameState = this.chess.make_engine_move();
            
            setTimeout(() => { 
                event.chessboard.setPosition(this.chess.to_fen(), true);

                if (this.gameState !== "Ongoing") {
                    this.handleGameOver();
                    return false;
                }

                this.board.enableMoveInput(this.handleInput.bind(this));
                this.info.innerText = "Your move";
            }, 10);
        }, 10);
    }

    handleGameOver() {
        this.info.innerText = `${this.gameState} - Game is resetting in 10 seconds...`;

        setTimeout(() => {
            this.chess.reset();
            this.board.setPosition(this.chess.to_fen());
            this.board.enableMoveInput(this.handleInput.bind(this));
            this.info.innerText = "Your move";
        }, 10000);
    }

    handleInput(event) {
        switch (event.type) {
            case "validateMoveInput":
                this.validateMoveInput(event);
                break;
            case "moveInputFinished":
                this.handleMoveInput(event);
                break;
            default:
                return true;
                break;
        }
    }
}

init().then(() => {
    const app = new App();
});
