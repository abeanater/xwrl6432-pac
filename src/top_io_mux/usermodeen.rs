#[doc = "Register `USERMODEEN` reader"]
pub type R = crate::R<UsermodeenSpec>;
#[doc = "Register `USERMODEEN` writer"]
pub type W = crate::W<UsermodeenSpec>;
#[doc = "Field `USERMODEEN` reader - 31:0\\]
Write 0XADADADAD to enable user mode write access to IO CFG space"]
pub type UsermodeenR = crate::FieldReader<u32>;
#[doc = "Field `USERMODEEN` writer - 31:0\\]
Write 0XADADADAD to enable user mode write access to IO CFG space"]
pub type UsermodeenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Write 0XADADADAD to enable user mode write access to IO CFG space"]
    #[inline(always)]
    pub fn usermodeen(&self) -> UsermodeenR {
        UsermodeenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Write 0XADADADAD to enable user mode write access to IO CFG space"]
    #[inline(always)]
    #[must_use]
    pub fn usermodeen(&mut self) -> UsermodeenW<UsermodeenSpec> {
        UsermodeenW::new(self, 0)
    }
}
#[doc = "USERMODEEN\n\nYou can [`read`](crate::Reg::read) this register and get [`usermodeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usermodeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsermodeenSpec;
impl crate::RegisterSpec for UsermodeenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usermodeen::R`](R) reader structure"]
impl crate::Readable for UsermodeenSpec {}
#[doc = "`write(|w| ..)` method takes [`usermodeen::W`](W) writer structure"]
impl crate::Writable for UsermodeenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USERMODEEN to value 0"]
impl crate::Resettable for UsermodeenSpec {
    const RESET_VALUE: u32 = 0;
}
