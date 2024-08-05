#[doc = "Register `QSUM4LSB` reader"]
pub type R = crate::R<Qsum4lsbSpec>;
#[doc = "Register `QSUM4LSB` writer"]
pub type W = crate::W<Qsum4lsbSpec>;
#[doc = "Field `QSUM4LSB` reader - 31:0\\]
Refer ISUM1LSB"]
pub type Qsum4lsbR = crate::FieldReader<u32>;
#[doc = "Field `QSUM4LSB` writer - 31:0\\]
Refer ISUM1LSB"]
pub type Qsum4lsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn qsum4lsb(&self) -> Qsum4lsbR {
        Qsum4lsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    #[must_use]
    pub fn qsum4lsb(&mut self) -> Qsum4lsbW<Qsum4lsbSpec> {
        Qsum4lsbW::new(self, 0)
    }
}
#[doc = "QSUM4LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum4lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum4lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qsum4lsbSpec;
impl crate::RegisterSpec for Qsum4lsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qsum4lsb::R`](R) reader structure"]
impl crate::Readable for Qsum4lsbSpec {}
#[doc = "`write(|w| ..)` method takes [`qsum4lsb::W`](W) writer structure"]
impl crate::Writable for Qsum4lsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSUM4LSB to value 0"]
impl crate::Resettable for Qsum4lsbSpec {
    const RESET_VALUE: u32 = 0;
}
