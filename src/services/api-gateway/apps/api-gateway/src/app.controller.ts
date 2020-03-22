import { Controller, Get, Logger } from '@nestjs/common';
import { AppService } from './app.service';

@Controller()
export class AppController {
  private logger = new Logger('AppController');

  constructor(private readonly appService: AppService) {}

  // TODO: didn't manage to type return with Observable (fail at compile)
  @Get()
  getAuthor() {
    this.logger.log('Getting an author from API Gateway');
    return this.appService.getAuthor();
  }
}
