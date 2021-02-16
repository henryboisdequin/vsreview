import {
  BaseEntity,
  Column,
  Entity,
  OneToMany,
  PrimaryGeneratedColumn,
} from "typeorm";
import { Question } from "./Question";
import { Like } from "./Like";
import { Answer } from "./Answer";

@Entity()
export class User extends BaseEntity {
  @PrimaryGeneratedColumn()
  id: number;

  @Column("text")
  name: string;

  @Column("text")
  profilePicture: string;

  @Column("text", { unique: true })
  githubId: string;

  @OneToMany(() => Question, (q) => q.creator)
  questions: Promise<Question[]>;

  @OneToMany(() => Question, (q) => q.creator)
  answers: Promise<Answer[]>;

  @OneToMany(() => Like, (l) => l.user)
  likes: Promise<Like>;
}
