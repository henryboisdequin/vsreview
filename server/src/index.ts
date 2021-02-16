import cors from "cors";
import express from "express";
import jwt from "jsonwebtoken";
import passport from "passport";
import { Strategy as GitHubStrategy } from "passport-github";
import { join } from "path";
import "reflect-metadata";
import { createConnection } from "typeorm";
import { __prod__ } from "./consts";
import { User } from "./entities/User";
require("dotenv-safe").config();

const main = async () => {
  await createConnection({
    type: "postgres",
    database: "vsreview",
    entities: [join(__dirname, "./entities/*.*")],
    username: "postgres",
    password: "postgres",
    logging: !__prod__,
    synchronize: !__prod__,
  });

  const app = express();

  passport.serializeUser((user: any, done) => {
    done(null, user.accessToken);
  });
  app.use(passport.initialize());
  app.use(cors({ origin: "*" }));
  app.use(express.json());

  passport.use(
    new GitHubStrategy(
      {
        clientID: process.env.GITHUB_CLIENT_ID as string,
        clientSecret: process.env.GITHUB_CLIENT_SECRET as string,
        callbackURL: "http://localhost:3000/auth/github/callback",
      },
      async (_, __, profile, cb) => {
        let user = await User.findOne({ where: { githubId: profile.id } });
        if (user) {
          user.name = profile.displayName;
          user.profilePicture = profile.profileUrl;
          await user.save();
        } else {
          user = await User.create({
            name: profile.displayName,
            profilePicture: profile.profileUrl,
            githubId: profile.id,
          }).save();
        }
        cb(null, {
          accessToken: jwt.sign(
            { userId: user.id },
            process.env.ACCESS_TOKEN_SECRET as string,
            {
              expiresIn: "1y",
            }
          ),
        });
      }
    )
  );

  app.get("/auth/github", passport.authenticate("github", { session: false }));
  app.get(
    "/auth/github/callback",
    passport.authenticate("github", { session: false }),
    (req: any, res) => {
      res.redirect(`http://localhost:4000/auth/${req.user.accessToken}`);
    }
  );

  app.get("/me", async (req, res) => {
    const authHeader = req.headers.authorization;
    if (!authHeader) {
      res.send({ user: null });
      return;
    }

    const token = authHeader.split(" ")[1];
    if (!token) {
      res.send({ user: null });
      return;
    }

    let userId = "";

    try {
      const payload: any = jwt.verify(
        token,
        process.env.ACCESS_TOKEN_SECRET as string
      );
      userId = payload.userId;
    } catch (err) {
      res.send({ user: null });
      return;
    }

    if (!userId) {
      res.send({ user: null });
      return;
    }

    const user = await User.findOne(userId);

    res.send({ user });
  });

  app.get("/", (_, res) => {
    res.send("Hello VSReview!");
  });

  app.listen(4000, () => {
    console.log("🚀 Server is starting on localhost:4000");
  });
};

main();
