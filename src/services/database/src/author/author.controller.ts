import { Controller, Logger } from '@nestjs/common';
import { GrpcMethod } from '@nestjs/microservices';
import { AuthorService } from './author.service';
import { Author } from './interfaces/author';
import { AuthorById } from './interfaces/author-by-id';

@Controller()
export class AuthorController {
  private logger = new Logger('AuthorController');

  constructor(private readonly appService: AuthorService) {}

  @GrpcMethod('AuthorService', 'FindOneById')
  findOneById(authorById: AuthorById): Author {
    this.logger.log(`Requesting authorwith id ${authorById.id.toString()}`);
    return this.appService.findOneById(authorById.id);
  }
}
