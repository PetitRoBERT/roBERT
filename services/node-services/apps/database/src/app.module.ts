import { Module } from '@nestjs/common';
import { MongooseModule } from '@nestjs/mongoose';
import { AuthorController } from './author/author.controller';
import { AuthorService } from './author/author.service';
import { AuthorModule } from './author/author.module';

const server = '127.0.0.1:27017';
const database = 'devpetitrobert';
@Module({
  imports: [AuthorModule, MongooseModule.forRoot(`mongodb://${server}/${database}`, { useNewUrlParser: true, useUnifiedTopology: true })],
  controllers: [AuthorController],
  providers: [AuthorService],
})
export class AppModule { }
