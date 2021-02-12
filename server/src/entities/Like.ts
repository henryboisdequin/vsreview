import { BaseEntity, Column, Entity, ManyToOne, PrimaryColumn } from "typeorm";
import { Question } from "./Question";
import { User } from "./User";

@Entity()
export class Like extends BaseEntity {
  @PrimaryColumn()
  userId: number;

  @Column({ type: "int" })
  value: number;

  @ManyToOne(() => User, (user) => user.likes)
  user: User;

  @PrimaryColumn()
  postId: number;

  @ManyToOne(() => Question, (q) => q.likes, {
    onDelete: "CASCADE",
  })
  question: Promise<Question>;
}
