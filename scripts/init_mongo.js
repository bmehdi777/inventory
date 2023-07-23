db.getSiblingDB("admin").auth(
	process.env.MONGO_INITDB_ROOT_USERNAME,
	process.env.MONGO_INITDB_ROOT_PASSWORD
);
db.createUser({
	user: process.env.MONGO_USER,
	pwd: process.env.MONGO_PASSWORD,
	roles: [{ role: "readWrite", db: process.env.MONGO_INITDB_DATABASE }],
});
db.createCollection("users");
db.users.createIndex({username: 1}, {unique: true})
db.createCollection("product");
db.createCollection("home");
