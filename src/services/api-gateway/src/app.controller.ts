import { Controller, Get, Logger } from '@nestjs/common';
import { AppService } from './app.service';
import { Observable } from 'rxjs';
import { Author } from './author/interfaces/author';

@Controller()
export class AppController {

  private logger = new Logger('AppController');

  constructor(private readonly appService: AppService) {}

  @Get()
  getAuthor(): Observable<Author> {
    this.logger.log('Getting an author from API Gateway');
    return this.appService.getAuthor();
  }
}
