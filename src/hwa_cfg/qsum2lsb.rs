#[doc = "Register `QSUM2LSB` reader"]
pub type R = crate::R<Qsum2lsbSpec>;
#[doc = "Register `QSUM2LSB` writer"]
pub type W = crate::W<Qsum2lsbSpec>;
#[doc = "Field `QSUM2LSB` reader - 31:0\\]
Refer ISUM1LSB"]
pub type Qsum2lsbR = crate::FieldReader<u32>;
#[doc = "Field `QSUM2LSB` writer - 31:0\\]
Refer ISUM1LSB"]
pub type Qsum2lsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    pub fn qsum2lsb(&self) -> Qsum2lsbR {
        Qsum2lsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Refer ISUM1LSB"]
    #[inline(always)]
    #[must_use]
    pub fn qsum2lsb(&mut self) -> Qsum2lsbW<Qsum2lsbSpec> {
        Qsum2lsbW::new(self, 0)
    }
}
#[doc = "QSUM2LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`qsum2lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qsum2lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qsum2lsbSpec;
impl crate::RegisterSpec for Qsum2lsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qsum2lsb::R`](R) reader structure"]
impl crate::Readable for Qsum2lsbSpec {}
#[doc = "`write(|w| ..)` method takes [`qsum2lsb::W`](W) writer structure"]
impl crate::Writable for Qsum2lsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSUM2LSB to value 0"]
impl crate::Resettable for Qsum2lsbSpec {
    const RESET_VALUE: u32 = 0;
}
