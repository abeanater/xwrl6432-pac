#[doc = "Register `QSUM1LSB` reader"]
pub type R = crate::R<Qsum1lsbSpec>;
#[doc = "Register `QSUM1LSB` writer"]
pub type W = crate::W<Qsum1lsbSpec>;
#[doc = "Field `QSUM1LSB` reader - 31:0\\]
Refer ISUM1LSB"]
pub type Qsum1lsbR = crate::FieldReader<u32>;
#[doc = "Field `QSUM1LSB` writer - 31:0\\]
Refer ISUM1LSB"]
pub type Qsum1lsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn qsum1lsb(&self) -> Qsum1lsbR {
        Qsum1lsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    #[must_use]
    pub fn qsum1lsb(&mut self) -> Qsum1lsbW<Qsum1lsbSpec> {
        Qsum1lsbW::new(self, 0)
    }
}
#[doc = "QSUM1LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum1lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum1lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qsum1lsbSpec;
impl crate::RegisterSpec for Qsum1lsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qsum1lsb::R`](R) reader structure"]
impl crate::Readable for Qsum1lsbSpec {}
#[doc = "`write(|w| ..)` method takes [`qsum1lsb::W`](W) writer structure"]
impl crate::Writable for Qsum1lsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSUM1LSB to value 0"]
impl crate::Resettable for Qsum1lsbSpec {
    const RESET_VALUE: u32 = 0;
}
