import { Module } from '@nestjs/common';
import { AuthorController } from './author/author.controller';
import { AuthorService } from './author/author.service';

@Module({
  imports: [],
  controllers: [AuthorController],
  providers: [AuthorService],
})
export class AppModule {}
