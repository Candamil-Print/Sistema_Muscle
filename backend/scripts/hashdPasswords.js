require("dotenv").config();
const bcrypt = require("bcrypt");
const { Usuario } = require("../models");

const ejecutar = async () => {
    try {

        const usuarios = await Usuario.findAll();

        for (const usuario of usuarios) {

            const hash = await bcrypt.hash(usuario.password, 10);

            await usuario.update({ password: hash });
        }

        console.log("Passwords encrypted successfully");
        process.exit();

    } catch (error) {
        console.error("Error hashing passwords:", error);
        process.exit(1);
    }
};

ejecutar();
