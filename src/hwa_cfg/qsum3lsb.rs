#[doc = "Register `QSUM3LSB` reader"]
pub type R = crate::R<Qsum3lsbSpec>;
#[doc = "Register `QSUM3LSB` writer"]
pub type W = crate::W<Qsum3lsbSpec>;
#[doc = "Field `QSUM3LSB` reader - 31:0\\]
Refer ISUM1LSB"]
pub type Qsum3lsbR = crate::FieldReader<u32>;
#[doc = "Field `QSUM3LSB` writer - 31:0\\]
Refer ISUM1LSB"]
pub type Qsum3lsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn qsum3lsb(&self) -> Qsum3lsbR {
        Qsum3lsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    #[must_use]
    pub fn qsum3lsb(&mut self) -> Qsum3lsbW<Qsum3lsbSpec> {
        Qsum3lsbW::new(self, 0)
    }
}
#[doc = "QSUM3LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum3lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum3lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qsum3lsbSpec;
impl crate::RegisterSpec for Qsum3lsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qsum3lsb::R`](R) reader structure"]
impl crate::Readable for Qsum3lsbSpec {}
#[doc = "`write(|w| ..)` method takes [`qsum3lsb::W`](W) writer structure"]
impl crate::Writable for Qsum3lsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSUM3LSB to value 0"]
impl crate::Resettable for Qsum3lsbSpec {
    const RESET_VALUE: u32 = 0;
}
