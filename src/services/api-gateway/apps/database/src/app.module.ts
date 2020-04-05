import { Module } from '@nestjs/common';
import { AuthorController } from './author/author.controller';
import { AuthorService } from './author/author.service';
import { AuthorModule } from './author/author.module';

@Module({
  imports: [AuthorModule],
  controllers: [AuthorController],
  providers: [AuthorService],
})
export class AppModule {}
